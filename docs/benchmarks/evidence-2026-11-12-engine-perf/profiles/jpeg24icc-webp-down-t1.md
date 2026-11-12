# prof/jpeg24icc-webp-down-t1.json.gz: total CPU weight 927091 (thread CPU delta units, us)

## Self (top 40)
| % | function |
| ---: | --- |
| 36.5 | `<<moxcms::cicp::TransferCharacteristics>::extended_linear_tristimulus::{closure#11} as ops::function::FnOnce<(moxcms::rgb::Rgb<f32>,)>>::call_once` |
| 11.6 | `<moxcms::conversions::rgbxyz_float::TransformShaperRgbFloatInOut<f32, 0, 0> as moxcms::transform::TransformExecutor<f32>>::transform` |
| 7.7 | `jpeg::jdhuff::decode_mcu` |
| 5.8 | `ferrisma_kernels::dispatch::neon::horizontal_rgba` |
| 4.2 | `jpeg::jdhuff::HUFF_DECODE_FAST` |
| 3.2 | `webp::dsp::enc_neon::load4x4_neon` |
| 3.1 | `webp::dsp::cost_neon::get_residual_cost` |
| 2.8 | `ferrisma::fast_execute::source_row_to_linear::<3>` |
| 2.1 | `webp::enc::token_enc::vp8_emit_tokens` |
| 1.8 | `webp::dsp::enc_neon::disto4x4_neon` |
| 1.7 | `ferrisma::fast_execute::quantize_row::<3>` |
| 1.7 | `webp::dsp::enc_neon::i_transform_one_neon` |
| 1.6 | `<moxcms::trc::ToneCurveEvaluatorLinear as moxcms::trc::ToneCurveEvaluator>::evaluate_tristimulus` |
| 1.5 | `ferrisma_kernels::dispatch::neon::axpy` |
| 1.2 | `<moxcms::trc::ToneCurveCicpEvaluator as moxcms::trc::ToneCurveEvaluator>::evaluate_tristimulus` |
| 1.1 | `webp::enc::quant_enc::vp8_decimate` |
| 1.1 | `webp::dsp::enc_neon::f_transform_neon` |
| 1.0 | `<ferrisma::cms::IccConversion as ferrisma::decoder::RowConversion>::convert` |
| 0.8 | `jpeg::jidctint_neon::jsimd_idct_islow_neon` |
| 0.7 | `webp::dsp::cpu::features` |
| 0.7 | `webp::enc::token_enc::vp8_record_coeff_tokens` |
| 0.6 | `webp::enc::picture_csp_enc::sum4` |
| 0.6 | `read` |
| 0.6 | `webp::dsp::enc_neon::quantize_block_neon` |
| 0.6 | `jpeg::jdcolext_neon::jsimd_ycc_rgb_convert_neon::<0, 1, 2, -1, 3>` |
| 0.5 | `jpeg::jdsample_neon::jsimd_h2v2_fancy_upsample_neon` |
| 0.5 | `jpeg::jdcoefct::decompress_onepass` |
| 0.4 | `webp::dsp::enc::vp8_collect_histogram` |
| 0.4 | `webp::enc::quant_enc::reconstruct_intra4` |
| 0.3 | `jpeg::jidctint_neon::jsimd_idct_islow_pass2_regular` |
| 0.3 | `webp::enc::picture_csp_enc::linear_to_gamma` |
| 0.3 | `webp::enc::picture_csp_enc::accumulate_rgb` |
| 0.2 | `webp::dsp::enc::vp8_enc_quantize2_blocks` |
| 0.2 | `jpeg::jsimd::jsimd_idct_islow` |
| 0.2 | `webp::enc::tree_enc::vp8_code_intra_modes` |
| 0.2 | `webp::enc::cost_enc::vp8_get_cost_luma16` |
| 0.2 | `<ferrisma::pipeline::Sink>::write_row` |
| 0.1 | `webp::enc::iterator_enc::import_block` |
| 0.1 | `_platform_memmove` |
| 0.1 | `libsystem_m.dylib+?` |

## Inclusive (top 40)
| % | function |
| ---: | --- |
| 100.0 | `ferrisma::cli::run` |
| 100.0 | `ferrisma::main` |
| 100.0 | `std::sys::backtrace::__rust_begin_short_backtrace::<fn() -> result::Result<std::process::ExitCode, boxed::Box<dyn error::Error>>, result::Result<std::process::ExitCode, boxed::Box<dyn error::Error>>>` |
| 100.0 | `std::rt::lang_start::<result::Result<..>>::{closure#0}` |
| 100.0 | `std::rt::lang_start_internal` |
| 100.0 | `main` |
| 100.0 | `start` |
| 99.3 | `ferrisma::pipeline::run` |
| 79.0 | `ferrisma::fast_execute::execute_fast_srgb8::<ferrisma::pipeline::run::{closure#1}>` |
| 78.8 | `<ferrisma::codecs::jpeg::JpegSource as ferrisma::decoder::DecodeSource>::read_region` |
| 78.8 | `<ferrisma::cms::IccSrgbSource as ferrisma::decoder::DecodeSource>::read_region` |
| 63.9 | `ferrisma::fast_execute::run_sequential::<3, ferrisma::pipeline::run::{closure#1}>::{closure#0}` |
| 63.9 | `<ferrisma::cms::IccSrgbSource as ferrisma::decoder::DecodeSource>::read_region::{closure#1}` |
| 54.6 | `ferrisma::fast_execute::source_row_to_linear::<3>` |
| 51.8 | `<ferrisma::cms::IccConversion as ferrisma::decoder::RowConversion>::convert` |
| 50.9 | `<moxcms::conversions::rgbxyz_float::TransformShaperRgbFloatInOut<f32, 0, 0> as moxcms::transform::TransformExecutor<f32>>::transform` |
| 36.5 | `<<moxcms::cicp::TransferCharacteristics>::extended_linear_tristimulus::{closure#11} as ops::function::FnOnce<(moxcms::rgb::Rgb<f32>,)>>::call_once` |
| 20.1 | `webp::enc::webp_enc::webp_encode` |
| 20.1 | `<webp::api::encode::Picture>::encode_with` |
| 20.1 | `<ferrisma::webp_sink::WebpSink>::finish` |
| 20.1 | `<ferrisma::pipeline::Sink>::finish` |
| 16.9 | `webp::enc::frame_enc::vp8_enc_token_loop` |
| 14.9 | `jpeg::jdmainct::process_data_context_main` |
| 14.9 | `jpeg::jdapistd::jpeg_read_scanlines` |
| 14.9 | `<jpeg::decode::Scanlines>::read_rows` |
| 13.9 | `webp::enc::quant_enc::vp8_decimate` |
| 13.8 | `jpeg::jdcoefct::decompress_onepass` |
| 11.9 | `jpeg::jdhuff::decode_mcu` |
| 5.8 | `ferrisma_kernels::dispatch::neon::horizontal_rgba` |
| 5.8 | `<ferrisma_kernels::Kernels>::horizontal_rgba` |
| 4.2 | `jpeg::jdhuff::HUFF_DECODE_FAST` |
| 4.0 | `webp::enc::quant_enc::reconstruct_intra4` |
| 3.2 | `webp::dsp::enc_neon::load4x4_neon` |
| 3.1 | `webp::dsp::cost_neon::get_residual_cost` |
| 2.7 | `webp::dsp::enc_neon::f_transform_neon` |
| 2.2 | `webp::enc::token_enc::vp8_emit_tokens` |
| 1.8 | `webp::dsp::enc_neon::disto4x4_neon` |
| 1.7 | `ferrisma::fast_execute::quantize_row::<3>` |
| 1.7 | `webp::dsp::enc_neon::i_transform_one_neon` |
| 1.7 | `webp::enc::analysis_enc::do_segments_job` |

## Stage shares (leaf-to-root first match)
| stage | % CPU | top self functions in stage |
| --- | ---: | --- |
| icc | 51.8 | `<<moxcms::cicp::TransferCharacteristics>::extended_linear_tristimulus:` 36.5; `<moxcms::conversions::rgbxyz_float::TransformShaperRgbFloatInOut<f32, ` 11.6; `<moxcms::trc::ToneCurveEvaluatorLinear as moxcms::trc::ToneCurveEvalua` 1.6; `<moxcms::trc::ToneCurveCicpEvaluator as moxcms::trc::ToneCurveEvaluato` 1.2 |
| webp-encode | 17.9 | `webp::dsp::enc_neon::load4x4_neon` 3.2; `webp::dsp::cost_neon::get_residual_cost` 3.1; `webp::enc::token_enc::vp8_emit_tokens` 2.1; `webp::dsp::enc_neon::disto4x4_neon` 1.8 |
| jpeg-decode | 14.9 | `jpeg::jdhuff::decode_mcu` 7.7; `jpeg::jdhuff::HUFF_DECODE_FAST` 4.2; `jpeg::jidctint_neon::jsimd_idct_islow_neon` 0.8; `jpeg::jdcolext_neon::jsimd_ycc_rgb_convert_neon::<0, 1, 2, -1, 3>` 0.6 |
| resize | 7.4 | `ferrisma_kernels::dispatch::neon::horizontal_rgba` 5.8; `ferrisma_kernels::dispatch::neon::axpy` 1.5; `__sysctl` 0.1 |
| linearize | 2.8 | `ferrisma::fast_execute::source_row_to_linear::<3>` 2.8 |
| quantize | 1.7 | `ferrisma::fast_execute::quantize_row::<3>` 1.7 |
| rgb2yuv | 1.4 | `webp::enc::picture_csp_enc::sum4` 0.6; `webp::enc::picture_csp_enc::linear_to_gamma` 0.3; `webp::enc::picture_csp_enc::accumulate_rgb` 0.3; `webp::enc::picture_csp_enc::convert_row_to_y` 0.1 |
| engine-pipeline | 1.3 | `read` 0.6; `<ferrisma::pipeline::Sink>::write_row` 0.2; `libsystem_m.dylib+?` 0.1; `slice::sort::unstable::quicksort::quicksort::<u32, <u32 as cmp::Partia` 0.1 |
| webp-decode | 0.8 | `webp::dsp::cpu::features` 0.7; `webp::utils::bit_writer_utils::bit_writer_resize` 0.1; `webp::utils::bit_writer_utils::flush` 0.0; `webp::dsp::yuv_neon::convert_rgba32_to_uv` 0.0 |
