# prof/jpeg24-webp-none-t1.json.gz: total CPU weight 4867652 (thread CPU delta units, us)

## Self (top 40)
| % | function |
| ---: | --- |
| 12.3 | `webp::dsp::enc_neon::load4x4_neon` |
| 9.8 | `webp::dsp::enc_neon::disto4x4_neon` |
| 8.8 | `webp::enc::quant_enc::vp8_decimate` |
| 7.9 | `webp::dsp::enc_neon::f_transform_neon` |
| 7.8 | `webp::dsp::cost_neon::get_residual_cost` |
| 5.5 | `webp::dsp::enc_neon::i_transform_one_neon` |
| 4.9 | `webp::dsp::enc_neon::quantize_block_neon` |
| 4.3 | `ferrisma::fast_execute::quantize_row::<3>` |
| 3.9 | `webp::enc::token_enc::vp8_emit_tokens` |
| 2.8 | `webp::dsp::cpu::features` |
| 2.4 | `webp::enc::token_enc::vp8_record_coeff_tokens` |
| 2.1 | `webp::dsp::enc::vp8_collect_histogram` |
| 2.1 | `webp::dsp::enc::vp8_enc_quantize2_blocks` |
| 2.0 | `webp::enc::quant_enc::reconstruct_intra4` |
| 1.8 | `webp::enc::picture_csp_enc::sum4` |
| 1.7 | `ferrisma_kernels::dispatch::neon::horizontal_rgba` |
| 1.6 | `jpeg::jdhuff::decode_mcu` |
| 1.3 | `libsystem_malloc.dylib+?` |
| 1.2 | `jpeg::jdhuff::HUFF_DECODE_FAST` |
| 1.1 | `webp::enc::picture_csp_enc::convert_row_to_y` |
| 1.1 | `ferrisma::fast_execute::source_row_to_linear::<3>` |
| 1.0 | `webp::enc::cost_enc::vp8_get_cost_luma16` |
| 0.8 | `webp::enc::cost_enc::vp8_get_cost_uv` |
| 0.8 | `webp::enc::quant_enc::reconstruct_uv` |
| 0.8 | `webp::enc::picture_csp_enc::linear_to_gamma` |
| 0.7 | `webp::enc::quant_enc::reconstruct_intra16` |
| 0.6 | `_platform_memmove` |
| 0.6 | `webp::enc::tree_enc::vp8_code_intra_modes` |
| 0.6 | `__bzero` |
| 0.5 | `webp::dsp::enc::true_motion` |
| 0.5 | `webp::enc::picture_csp_enc::import_yuva_from_rgba` |
| 0.5 | `<ferrisma::pipeline::Sink>::write_row` |
| 0.5 | `jpeg::jidctint_neon::jsimd_idct_islow_neon` |
| 0.5 | `webp::dsp::enc::vp8_set_histogram_data` |
| 0.4 | `ferrisma_kernels::dispatch::neon::axpy` |
| 0.3 | `<vec::Vec<u8> as vec::spec_from_iter_nested::SpecFromIterNested<u8, iter::adapters::flatten::FlatMap<..>>>::from_iter` |
| 0.3 | `webp::enc::iterator_enc::vp8_iterator_rotate_i4` |
| 0.3 | `webp::utils::bit_writer_utils::flush` |
| 0.3 | `webp::dsp::enc::vp8t_disto16x16` |
| 0.2 | `_platform_memset` |

## Inclusive (top 40)
| % | function |
| ---: | --- |
| 100.0 | `start` |
| 99.9 | `ferrisma::cli::run` |
| 99.9 | `ferrisma::main` |
| 99.9 | `std::sys::backtrace::__rust_begin_short_backtrace::<fn() -> result::Result<std::process::ExitCode, boxed::Box<dyn error::Error>>, result::Result<std::process::ExitCode, boxed::Box<dyn error::Error>>>` |
| 99.9 | `std::rt::lang_start::<result::Result<..>>::{closure#0}` |
| 99.9 | `std::rt::lang_start_internal` |
| 99.9 | `main` |
| 99.8 | `ferrisma::pipeline::run` |
| 87.2 | `<ferrisma::webp_sink::WebpSink>::finish` |
| 87.2 | `<ferrisma::pipeline::Sink>::finish` |
| 87.2 | `webp::enc::webp_enc::webp_encode` |
| 87.2 | `<webp::api::encode::Picture>::encode_with` |
| 77.4 | `webp::enc::frame_enc::vp8_enc_token_loop` |
| 67.9 | `webp::enc::quant_enc::vp8_decimate` |
| 18.4 | `webp::enc::quant_enc::reconstruct_intra4` |
| 14.1 | `webp::dsp::enc_neon::f_transform_neon` |
| 12.4 | `ferrisma::fast_execute::execute_fast_srgb8::<ferrisma::pipeline::run::{closure#1}>` |
| 12.3 | `<ferrisma::codecs::jpeg::JpegSource as ferrisma::decoder::DecodeSource>::read_region` |
| 12.3 | `webp::dsp::enc_neon::load4x4_neon` |
| 9.8 | `webp::dsp::enc_neon::disto4x4_neon` |
| 8.2 | `ferrisma::fast_execute::run_sequential::<3, ferrisma::pipeline::run::{closure#1}>::{closure#0}` |
| 7.8 | `webp::dsp::cost_neon::get_residual_cost` |
| 7.4 | `webp::enc::quant_enc::reconstruct_intra16` |
| 5.5 | `webp::dsp::enc_neon::i_transform_one_neon` |
| 5.0 | `webp::enc::picture_csp_enc::picture_argb_to_yuva` |
| 4.9 | `webp::dsp::enc_neon::quantize_block_neon` |
| 4.6 | `webp::enc::picture_csp_enc::import_yuva_from_rgba` |
| 4.3 | `webp::enc::token_enc::vp8_emit_tokens` |
| 4.3 | `webp::enc::quant_enc::reconstruct_uv` |
| 4.3 | `ferrisma::fast_execute::quantize_row::<3>` |
| 4.2 | `webp::enc::analysis_enc::do_segments_job` |
| 4.2 | `webp::enc::analysis_enc::vp8_enc_analyze` |
| 4.2 | `<jpeg::decode::Scanlines>::read_rows` |
| 4.2 | `webp::enc::cost_enc::vp8_get_cost_luma16` |
| 4.1 | `jpeg::jdmainct::process_data_context_main` |
| 4.1 | `jpeg::jdapistd::jpeg_read_scanlines` |
| 4.1 | `webp::dsp::enc::vp8_collect_histogram` |
| 3.7 | `jpeg::jdcoefct::decompress_onepass` |
| 3.5 | `webp::dsp::enc::vp8t_disto16x16` |
| 2.8 | `webp::dsp::cpu::features` |

## Stage shares (leaf-to-root first match)
| stage | % CPU | top self functions in stage |
| --- | ---: | --- |
| webp-encode | 77.8 | `webp::dsp::enc_neon::load4x4_neon` 12.3; `webp::dsp::enc_neon::disto4x4_neon` 9.8; `webp::enc::quant_enc::vp8_decimate` 8.8; `webp::dsp::enc_neon::f_transform_neon` 7.9 |
| rgb2yuv | 4.8 | `webp::enc::picture_csp_enc::sum4` 1.8; `webp::enc::picture_csp_enc::convert_row_to_y` 1.1; `webp::enc::picture_csp_enc::linear_to_gamma` 0.8; `webp::enc::picture_csp_enc::import_yuva_from_rgba` 0.5 |
| webp-decode | 4.6 | `webp::dsp::cpu::features` 2.8; `libsystem_malloc.dylib+?` 1.3; `webp::utils::bit_writer_utils::flush` 0.3; `webp::dsp::yuv_neon::convert_rgba32_to_uv` 0.1 |
| quantize | 4.3 | `ferrisma::fast_execute::quantize_row::<3>` 4.3 |
| jpeg-decode | 4.2 | `jpeg::jdhuff::decode_mcu` 1.6; `jpeg::jdhuff::HUFF_DECODE_FAST` 1.2; `jpeg::jidctint_neon::jsimd_idct_islow_neon` 0.5; `jpeg::jidctint_neon::jsimd_idct_islow_pass2_regular` 0.2 |
| resize | 2.2 | `ferrisma_kernels::dispatch::neon::horizontal_rgba` 1.7; `ferrisma_kernels::dispatch::neon::axpy` 0.4; `<ferrisma_kernels::Kernels>::horizontal_rgba` 0.0 |
| linearize | 1.1 | `ferrisma::fast_execute::source_row_to_linear::<3>` 1.1 |
| engine-pipeline | 1.0 | `<ferrisma::pipeline::Sink>::write_row` 0.5; `__bzero` 0.3; `read` 0.0; `ferrisma::quality::taps` 0.0 |
| other | 0.1 | `_kernelrpc_mach_vm_map_trap` 0.1; `libsystem_c.dylib+?` 0.0 |
