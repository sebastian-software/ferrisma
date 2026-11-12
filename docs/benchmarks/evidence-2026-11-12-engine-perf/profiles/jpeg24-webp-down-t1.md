# prof/jpeg24-webp-down-t1.json.gz: total CPU weight 769555 (thread CPU delta units, us)

## Self (top 40)
| % | function |
| ---: | --- |
| 18.8 | `ferrisma_kernels::dispatch::neon::horizontal_rgba` |
| 11.9 | `jpeg::jdhuff::decode_mcu` |
| 6.3 | `jpeg::jdhuff::HUFF_DECODE_FAST` |
| 5.1 | `ferrisma::fast_execute::source_row_to_linear::<3>` |
| 4.8 | `webp::dsp::enc_neon::load4x4_neon` |
| 4.0 | `webp::dsp::enc_neon::disto4x4_neon` |
| 3.9 | `ferrisma_kernels::dispatch::neon::axpy` |
| 3.5 | `webp::dsp::cost_neon::get_residual_cost` |
| 3.2 | `webp::enc::token_enc::vp8_emit_tokens` |
| 3.2 | `ferrisma::fast_execute::quantize_row::<3>` |
| 2.9 | `webp::enc::quant_enc::vp8_decimate` |
| 2.9 | `jpeg::jidctint_neon::jsimd_idct_islow_neon` |
| 2.3 | `webp::dsp::enc::vp8_collect_histogram` |
| 2.3 | `webp::dsp::enc_neon::f_transform_neon` |
| 1.8 | `webp::dsp::enc_neon::i_transform_one_neon` |
| 1.6 | `jpeg::jidctint_neon::jsimd_idct_islow_pass2_regular` |
| 1.6 | `webp::dsp::enc::vp8_enc_quantize2_blocks` |
| 1.6 | `webp::dsp::enc_neon::quantize_block_neon` |
| 1.5 | `jpeg::jdcolext_neon::jsimd_ycc_rgb_convert_neon::<0, 1, 2, -1, 3>` |
| 1.2 | `webp::enc::picture_csp_enc::sum4` |
| 1.1 | `webp::enc::token_enc::vp8_record_coeff_tokens` |
| 1.1 | `lsl::Allocator::AllocationMetadata::AllocationMetadata(lsl::Allocator::AllocationMetadata*, unsigned long long, unsigned long long, unsigned long long)` |
| 1.0 | `jpeg::jidctint_neon::jsimd_idct_islow_pass2_sparse` |
| 1.0 | `webp::dsp::cpu::features` |
| 0.8 | `webp::enc::picture_csp_enc::convert_row_to_y` |
| 0.8 | `_platform_memset` |
| 0.8 | `webp::enc::quant_enc::reconstruct_intra4` |
| 0.7 | `jpeg::jdsample_neon::jsimd_h2v2_fancy_upsample_neon` |
| 0.6 | `jpeg::jdcoefct::decompress_onepass` |
| 0.5 | `_platform_memmove` |
| 0.5 | `webp::enc::cost_enc::vp8_get_cost_luma16` |
| 0.5 | `webp::enc::tree_enc::vp8_code_intra_modes` |
| 0.5 | `webp::enc::picture_csp_enc::linear_to_gamma` |
| 0.4 | `webp::dsp::enc::true_motion` |
| 0.4 | `libsystem_m.dylib+?` |
| 0.4 | `ferrisma::fast_execute::run_sequential::<3, ferrisma::pipeline::run::{closure#1}>::{closure#0}` |
| 0.3 | `webp::enc::cost_enc::vp8_get_cost_uv` |
| 0.3 | `libsystem_malloc.dylib+?` |
| 0.3 | `<vec::Vec<u8> as vec::spec_from_iter_nested::SpecFromIterNested<u8, iter::adapters::flatten::FlatMap<..>>>::from_iter` |
| 0.3 | `webp::utils::bit_writer_utils::flush` |

## Inclusive (top 40)
| % | function |
| ---: | --- |
| 100.0 | `start` |
| 98.9 | `ferrisma::cli::run` |
| 98.9 | `ferrisma::main` |
| 98.9 | `std::sys::backtrace::__rust_begin_short_backtrace::<fn() -> result::Result<std::process::ExitCode, boxed::Box<dyn error::Error>>, result::Result<std::process::ExitCode, boxed::Box<dyn error::Error>>>` |
| 98.9 | `std::rt::lang_start::<result::Result<..>>::{closure#0}` |
| 98.9 | `std::rt::lang_start_internal` |
| 98.9 | `main` |
| 98.6 | `ferrisma::pipeline::run` |
| 60.0 | `ferrisma::fast_execute::execute_fast_srgb8::<ferrisma::pipeline::run::{closure#1}>` |
| 59.1 | `<ferrisma::codecs::jpeg::JpegSource as ferrisma::decoder::DecodeSource>::read_region` |
| 38.6 | `webp::enc::webp_enc::webp_encode` |
| 38.6 | `<webp::api::encode::Picture>::encode_with` |
| 38.6 | `<ferrisma::webp_sink::WebpSink>::finish` |
| 38.6 | `<ferrisma::pipeline::Sink>::finish` |
| 31.4 | `ferrisma::fast_execute::run_sequential::<3, ferrisma::pipeline::run::{closure#1}>::{closure#0}` |
| 30.9 | `webp::enc::frame_enc::vp8_enc_token_loop` |
| 27.7 | `<jpeg::decode::Scanlines>::read_rows` |
| 27.6 | `jpeg::jdmainct::process_data_context_main` |
| 27.6 | `jpeg::jdapistd::jpeg_read_scanlines` |
| 25.7 | `webp::enc::quant_enc::vp8_decimate` |
| 25.0 | `jpeg::jdcoefct::decompress_onepass` |
| 18.8 | `ferrisma_kernels::dispatch::neon::horizontal_rgba` |
| 18.8 | `<ferrisma_kernels::Kernels>::horizontal_rgba` |
| 18.2 | `jpeg::jdhuff::decode_mcu` |
| 6.8 | `webp::enc::quant_enc::reconstruct_intra4` |
| 6.3 | `jpeg::jdhuff::HUFF_DECODE_FAST` |
| 5.4 | `jpeg::jidctint_neon::jsimd_idct_islow_neon` |
| 5.1 | `ferrisma::fast_execute::source_row_to_linear::<3>` |
| 4.8 | `webp::dsp::enc_neon::load4x4_neon` |
| 4.3 | `webp::dsp::enc_neon::f_transform_neon` |
| 4.2 | `webp::enc::analysis_enc::vp8_enc_analyze` |
| 4.1 | `webp::enc::analysis_enc::do_segments_job` |
| 4.0 | `webp::dsp::enc_neon::disto4x4_neon` |
| 3.9 | `ferrisma_kernels::dispatch::neon::axpy` |
| 3.9 | `<ferrisma_kernels::Kernels>::axpy` |
| 3.6 | `webp::enc::token_enc::vp8_emit_tokens` |
| 3.5 | `webp::dsp::enc::vp8_collect_histogram` |
| 3.5 | `webp::dsp::cost_neon::get_residual_cost` |
| 3.2 | `ferrisma::fast_execute::quantize_row::<3>` |
| 3.0 | `webp::enc::picture_csp_enc::picture_argb_to_yuva` |

## Stage shares (leaf-to-root first match)
| stage | % CPU | top self functions in stage |
| --- | ---: | --- |
| webp-encode | 34.3 | `webp::dsp::enc_neon::load4x4_neon` 4.8; `webp::dsp::enc_neon::disto4x4_neon` 4.0; `webp::dsp::cost_neon::get_residual_cost` 3.5; `webp::enc::token_enc::vp8_emit_tokens` 3.2 |
| jpeg-decode | 27.7 | `jpeg::jdhuff::decode_mcu` 11.9; `jpeg::jdhuff::HUFF_DECODE_FAST` 6.3; `jpeg::jidctint_neon::jsimd_idct_islow_neon` 2.9; `jpeg::jidctint_neon::jsimd_idct_islow_pass2_regular` 1.6 |
| resize | 22.7 | `ferrisma_kernels::dispatch::neon::horizontal_rgba` 18.8; `ferrisma_kernels::dispatch::neon::axpy` 3.9 |
| linearize | 5.1 | `ferrisma::fast_execute::source_row_to_linear::<3>` 5.1 |
| quantize | 3.2 | `ferrisma::fast_execute::quantize_row::<3>` 3.2 |
| rgb2yuv | 3.0 | `webp::enc::picture_csp_enc::sum4` 1.2; `webp::enc::picture_csp_enc::convert_row_to_y` 0.8; `webp::enc::picture_csp_enc::linear_to_gamma` 0.5; `<vec::Vec<u8> as vec::spec_from_iter_nested::SpecFromIterNested<u8, it` 0.3 |
| engine-pipeline | 1.7 | `libsystem_m.dylib+?` 0.4; `ferrisma::fast_execute::run_sequential::<3, ferrisma::pipeline::run::{` 0.4; `ferrisma::quality::taps` 0.2; `read` 0.1 |
| webp-decode | 1.3 | `webp::dsp::cpu::features` 1.0; `webp::utils::bit_writer_utils::flush` 0.3; `webp::utils::bit_writer_utils::bit_writer_resize` 0.1 |
| other | 1.1 | `lsl::Allocator::AllocationMetadata::AllocationMetadata(lsl::Allocator:` 1.1 |
