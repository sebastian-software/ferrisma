# prof/jpeg12o6-webp-down-t1.json.gz: total CPU weight 1208323 (thread CPU delta units, us)

## Self (top 40)
| % | function |
| ---: | --- |
| 36.9 | `<<moxcms::cicp::TransferCharacteristics>::extended_linear_tristimulus::{closure#11} as ops::function::FnOnce<(moxcms::rgb::Rgb<f32>,)>>::call_once` |
| 5.0 | `<moxcms::conversions::rgbxyz_float::TransformShaperRgbFloatInOut<f32, 0, 0> as moxcms::transform::TransformExecutor<f32>>::transform` |
| 4.9 | `ferrisma::orientation::gather_row::<u8>` |
| 3.9 | `ferrisma_kernels::dispatch::neon::horizontal_rgba` |
| 3.8 | `webp::dsp::enc_neon::disto4x4_neon` |
| 3.7 | `webp::dsp::enc_neon::load4x4_neon` |
| 3.0 | `webp::dsp::cost_neon::get_residual_cost` |
| 2.7 | `jpeg::jdhuff::decode_mcu` |
| 2.6 | `webp::enc::quant_enc::vp8_decimate` |
| 2.1 | `<moxcms::trc::ToneCurveEvaluatorLinear as moxcms::trc::ToneCurveEvaluator>::evaluate_tristimulus` |
| 2.1 | `webp::dsp::enc_neon::i_transform_one_neon` |
| 2.0 | `ferrisma::fast_execute::source_row_to_linear::<3>` |
| 1.9 | `webp::dsp::enc_neon::f_transform_neon` |
| 1.9 | `jpeg::jdhuff::HUFF_DECODE_FAST` |
| 1.7 | `ferrisma::fast_execute::quantize_row::<3>` |
| 1.6 | `webp::enc::token_enc::vp8_emit_tokens` |
| 1.6 | `ferrisma_kernels::dispatch::neon::axpy` |
| 1.6 | `webp::dsp::enc_neon::quantize_block_neon` |
| 1.6 | `<moxcms::trc::ToneCurveCicpEvaluator as moxcms::trc::ToneCurveEvaluator>::evaluate_tristimulus` |
| 1.3 | `webp::dsp::enc::vp8_collect_histogram` |
| 1.1 | `webp::enc::quant_enc::reconstruct_intra4` |
| 1.0 | `webp::dsp::cpu::features` |
| 0.9 | `jpeg::jidctint_neon::jsimd_idct_islow_neon` |
| 0.8 | `webp::enc::picture_csp_enc::sum4` |
| 0.7 | `jpeg::jidctint_neon::jsimd_idct_islow_pass2_regular` |
| 0.6 | `jpeg::jdcolext_neon::jsimd_ycc_rgb_convert_neon::<0, 1, 2, -1, 3>` |
| 0.5 | `webp::enc::picture_csp_enc::linear_to_gamma` |
| 0.5 | `read` |
| 0.5 | `webp::enc::token_enc::vp8_record_coeff_tokens` |
| 0.5 | `<ferrisma::cms::IccConversion as ferrisma::decoder::RowConversion>::convert` |
| 0.5 | `__bzero` |
| 0.5 | `_platform_memmove` |
| 0.4 | `webp::dsp::enc::vp8_enc_quantize2_blocks` |
| 0.4 | `webp::enc::cost_enc::vp8_get_cost_luma16` |
| 0.4 | `webp::enc::cost_enc::vp8_get_cost_uv` |
| 0.4 | `webp::enc::tree_enc::vp8_code_intra_modes` |
| 0.4 | `webp::dsp::enc::vp8_set_histogram_data` |
| 0.3 | `webp::enc::quant_enc::reconstruct_intra16` |
| 0.3 | `webp::dsp::enc::true_motion` |
| 0.3 | `ferrisma::fast_execute::run_sequential::<3, ferrisma::pipeline::run::{closure#1}>::{closure#0}` |

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
| 99.4 | `ferrisma::pipeline::run` |
| 69.1 | `ferrisma::fast_execute::execute_fast_srgb8::<ferrisma::pipeline::run::{closure#1}>` |
| 68.9 | `<ferrisma::orientation::OrientedSource as ferrisma::decoder::DecodeSource>::read_region` |
| 68.9 | `<ferrisma::cms::IccSrgbSource as ferrisma::decoder::DecodeSource>::read_region` |
| 55.9 | `ferrisma::fast_execute::run_sequential::<3, ferrisma::pipeline::run::{closure#1}>::{closure#0}` |
| 55.9 | `<ferrisma::cms::IccSrgbSource as ferrisma::decoder::DecodeSource>::read_region::{closure#1}` |
| 48.1 | `ferrisma::fast_execute::source_row_to_linear::<3>` |
| 46.2 | `<ferrisma::cms::IccConversion as ferrisma::decoder::RowConversion>::convert` |
| 45.7 | `<moxcms::conversions::rgbxyz_float::TransformShaperRgbFloatInOut<f32, 0, 0> as moxcms::transform::TransformExecutor<f32>>::transform` |
| 36.9 | `<<moxcms::cicp::TransferCharacteristics>::extended_linear_tristimulus::{closure#11} as ops::function::FnOnce<(moxcms::rgb::Rgb<f32>,)>>::call_once` |
| 30.3 | `<ferrisma::webp_sink::WebpSink>::finish` |
| 30.3 | `<ferrisma::pipeline::Sink>::finish` |
| 30.2 | `<webp::api::encode::Picture>::encode_with` |
| 30.2 | `webp::enc::webp_enc::webp_encode` |
| 25.4 | `webp::enc::frame_enc::vp8_enc_token_loop` |
| 22.4 | `webp::enc::quant_enc::vp8_decimate` |
| 7.6 | `<ferrisma::codecs::jpeg::JpegSource as ferrisma::decoder::DecodeSource>::read_region` |
| 7.5 | `jpeg::jdmainct::process_data_context_main` |
| 7.5 | `jpeg::jdapistd::jpeg_read_scanlines` |
| 7.5 | `<jpeg::decode::Scanlines>::read_rows` |
| 6.6 | `jpeg::jdcoefct::decompress_onepass` |
| 6.1 | `webp::enc::quant_enc::reconstruct_intra4` |
| 4.9 | `ferrisma::orientation::gather_row::<u8>` |
| 4.6 | `jpeg::jdhuff::decode_mcu` |
| 3.9 | `ferrisma_kernels::dispatch::neon::horizontal_rgba` |
| 3.9 | `<ferrisma_kernels::Kernels>::horizontal_rgba` |
| 3.8 | `webp::dsp::enc_neon::disto4x4_neon` |
| 3.7 | `webp::dsp::enc_neon::load4x4_neon` |
| 3.7 | `webp::dsp::enc_neon::f_transform_neon` |
| 3.0 | `webp::dsp::cost_neon::get_residual_cost` |
| 2.5 | `webp::enc::analysis_enc::do_segments_job` |
| 2.5 | `webp::enc::analysis_enc::vp8_enc_analyze` |
| 2.2 | `webp::dsp::enc::vp8_collect_histogram` |

## Stage shares (leaf-to-root first match)
| stage | % CPU | top self functions in stage |
| --- | ---: | --- |
| icc | 46.2 | `<<moxcms::cicp::TransferCharacteristics>::extended_linear_tristimulus:` 36.9; `<moxcms::conversions::rgbxyz_float::TransformShaperRgbFloatInOut<f32, ` 5.0; `<moxcms::trc::ToneCurveEvaluatorLinear as moxcms::trc::ToneCurveEvalua` 2.1; `<moxcms::trc::ToneCurveCicpEvaluator as moxcms::trc::ToneCurveEvaluato` 1.6 |
| webp-encode | 27.3 | `webp::dsp::enc_neon::disto4x4_neon` 3.8; `webp::dsp::enc_neon::load4x4_neon` 3.7; `webp::dsp::cost_neon::get_residual_cost` 3.0; `webp::enc::quant_enc::vp8_decimate` 2.6 |
| jpeg-decode | 7.5 | `jpeg::jdhuff::decode_mcu` 2.7; `jpeg::jdhuff::HUFF_DECODE_FAST` 1.9; `jpeg::jidctint_neon::jsimd_idct_islow_neon` 0.9; `jpeg::jidctint_neon::jsimd_idct_islow_pass2_regular` 0.7 |
| resize | 5.6 | `ferrisma_kernels::dispatch::neon::horizontal_rgba` 3.9; `ferrisma_kernels::dispatch::neon::axpy` 1.6; `<ferrisma_kernels::Kernels>::axpy` 0.0 |
| orientation | 5.1 | `ferrisma::orientation::gather_row::<u8>` 4.9; `_platform_memmove` 0.2 |
| linearize | 2.0 | `ferrisma::fast_execute::source_row_to_linear::<3>` 2.0 |
| rgb2yuv | 1.8 | `webp::enc::picture_csp_enc::sum4` 0.8; `webp::enc::picture_csp_enc::linear_to_gamma` 0.5; `webp::enc::picture_csp_enc::convert_row_to_y` 0.2; `webp::enc::picture_csp_enc::accumulate_rgb` 0.1 |
| quantize | 1.7 | `ferrisma::fast_execute::quantize_row::<3>` 1.7 |
| engine-pipeline | 1.7 | `read` 0.5; `__bzero` 0.4; `ferrisma::fast_execute::run_sequential::<3, ferrisma::pipeline::run::{` 0.3; `libsystem_m.dylib+?` 0.2 |
| webp-decode | 1.2 | `webp::dsp::cpu::features` 1.0; `webp::utils::bit_writer_utils::flush` 0.1; `webp::dsp::yuv_neon::convert_rgba32_to_uv` 0.1; `webp::dsp::dec_neon::transform_wht` 0.0 |
