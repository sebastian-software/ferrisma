# prof/avif24-webp-down-t1.json.gz: total CPU weight 1888712 (thread CPU delta units, us)

## Self (top 40)
| % | function |
| ---: | --- |
| 11.7 | `od_ec_decode_cdf_q15` |
| 11.6 | `avif::reformat::avifImageYUV8ToRGB8Color` |
| 6.8 | `av1_read_coeffs_txb` |
| 5.8 | `_platform_memset` |
| 5.4 | `ferrisma_kernels::dispatch::neon::horizontal_rgba` |
| 4.8 | `aom::av1__common__av1_loopfilter::set_lpf_parameters` |
| 3.2 | `__bzero` |
| 2.6 | `av1_inv_txfm_add_c` |
| 2.5 | `<u8 as c2rust_bitfields::FieldType>::get_field` |
| 2.2 | `webp::dsp::enc_neon::disto4x4_neon` |
| 2.0 | `webp::dsp::cost_neon::get_residual_cost` |
| 1.9 | `od_ec_decode_bool_q15` |
| 1.8 | `aom::aom_dsp__loopfilter::filter8` |
| 1.6 | `aom::av1__decoder__decodeframe::parse_decode_block` |
| 1.6 | `webp::dsp::enc_neon::f_transform_neon` |
| 1.4 | `webp::enc::token_enc::vp8_emit_tokens` |
| 1.4 | `webp::dsp::enc_neon::load4x4_neon` |
| 1.3 | `aom::av1__common__av1_loopfilter::get_filter_level` |
| 1.1 | `ferrisma::fast_execute::source_row_to_linear::<3>` |
| 1.1 | `webp::enc::quant_enc::vp8_decimate` |
| 1.1 | `av1_predict_intra_block` |
| 1.1 | `aom_lpf_vertical_4_c` |
| 1.0 | `aom_lpf_horizontal_4_c` |
| 1.0 | `ferrisma_kernels::dispatch::neon::axpy` |
| 1.0 | `av1_read_mode_info` |
| 1.0 | `aom_lpf_horizontal_8_c` |
| 0.9 | `webp::dsp::enc_neon::quantize_block_neon` |
| 0.8 | `webp::dsp::enc_neon::i_transform_one_neon` |
| 0.8 | `webp::enc::token_enc::vp8_record_coeff_tokens` |
| 0.8 | `cdef_find_dir_neon` |
| 0.8 | `aom::simd::av1__common__arm__highbd_inv_txfm_neon::idct8x8_neon` |
| 0.7 | `ferrisma::fast_execute::quantize_row::<3>` |
| 0.7 | `aom::aom_dsp__loopfilter::mb_lpf_vertical_edge_w` |
| 0.7 | `aom_lpf_vertical_8_c` |
| 0.7 | `aom::aom_dsp__loopfilter::filter6` |
| 0.7 | `aom::simd::av1__common__arm__highbd_inv_txfm_neon::inv_txfm2d_add_universe_neon` |
| 0.7 | `aom::aom_dsp__loopfilter::filter14` |
| 0.6 | `webp::dsp::cpu::features` |
| 0.6 | `aom::simd::av1__common__arm__highbd_inv_txfm_neon::idct16x16_neon` |
| 0.6 | `webp::dsp::enc::vp8_collect_histogram` |

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
| 99.7 | `ferrisma::pipeline::run` |
| 82.9 | `ferrisma::fast_execute::execute_fast_srgb8::<ferrisma::pipeline::run::{closure#1}>` |
| 82.5 | `<ferrisma::codecs::avif::AvifSource as ferrisma::decoder::DecodeSource>::read_region` |
| 73.7 | `<ferrisma_avif_read::Decoder>::decode_u8` |
| 62.1 | `aom_decode_frame_from_obus` |
| 62.1 | `aom::jump::value_scope::<i32, aom::av1__decoder__decoder::av1_receive_compressed_data::{closure#0}>` |
| 62.1 | `av1_receive_compressed_data` |
| 62.1 | `aom::av1__av1_dx_iface::frame_worker_hook` |
| 62.1 | `aom::aom_util__aom_thread::execute` |
| 62.1 | `aom::av1__av1_dx_iface::decoder_decode` |
| 62.1 | `aom_codec_decode` |
| 62.1 | `avif::codec_aom::aomCodecGetNextImage` |
| 62.1 | `avifDecoderNextImage` |
| 59.6 | `av1_decode_tg_tiles_and_wrapup` |
| 41.0 | `aom::av1__decoder__decodeframe::decode_partition` |
| 35.3 | `aom::av1__decoder__decodeframe::decode_token_recon_block` |
| 20.6 | `av1_read_coeffs_txb` |
| 16.7 | `webp::enc::webp_enc::webp_encode` |
| 16.7 | `<webp::api::encode::Picture>::encode_with` |
| 16.7 | `<ferrisma::webp_sink::WebpSink>::finish` |
| 16.7 | `<ferrisma::pipeline::Sink>::finish` |
| 16.5 | `av1_loop_filter_frame_mt` |
| 14.8 | `webp::enc::frame_enc::vp8_enc_token_loop` |
| 14.1 | `aom::av1__decoder__decodeframe::predict_and_reconstruct_intra_block` |
| 11.8 | `webp::enc::quant_enc::vp8_decimate` |
| 11.7 | `od_ec_decode_cdf_q15` |
| 11.6 | `avif::reformat::avifImageYUV8ToRGB8Color` |
| 11.6 | `avif::reformat::avifImageYUVToRGBImpl` |
| 11.6 | `avifImageYUVToRGB` |
| 11.0 | `av1_inverse_transform_block` |
| 10.7 | `av1_inv_txfm_add_c` |
| 8.5 | `av1_filter_block_plane_horz` |
| 8.5 | `av1_thread_loop_filter_rows` |

## Stage shares (leaf-to-root first match)
| stage | % CPU | top self functions in stage |
| --- | ---: | --- |
| av1-decode | 62.1 | `od_ec_decode_cdf_q15` 11.7; `av1_read_coeffs_txb` 6.8; `_platform_memset` 5.7; `aom::av1__common__av1_loopfilter::set_lpf_parameters` 4.8 |
| webp-encode | 15.1 | `webp::dsp::enc_neon::disto4x4_neon` 2.2; `webp::dsp::cost_neon::get_residual_cost` 2.0; `webp::dsp::enc_neon::f_transform_neon` 1.6; `webp::enc::token_enc::vp8_emit_tokens` 1.4 |
| yuv2rgb | 11.6 | `avif::reformat::avifImageYUV8ToRGB8Color` 11.6 |
| resize | 6.4 | `ferrisma_kernels::dispatch::neon::horizontal_rgba` 5.4; `ferrisma_kernels::dispatch::neon::axpy` 1.0; `<ferrisma_kernels::Kernels>::horizontal_rgba` 0.0 |
| engine-pipeline | 1.5 | `__bzero` 0.5; `read` 0.3; `ferrisma::quality::taps` 0.2; `libsystem_m.dylib+?` 0.1 |
| linearize | 1.1 | `ferrisma::fast_execute::source_row_to_linear::<3>` 1.1 |
| webp-decode | 0.9 | `webp::dsp::cpu::features` 0.6; `webp::utils::bit_writer_utils::flush` 0.1; `webp::utils::bit_writer_utils::bit_writer_resize` 0.1; `webp::dsp::dec_neon::transform_wht` 0.0 |
| rgb2yuv | 0.8 | `webp::enc::picture_csp_enc::sum4` 0.4; `webp::enc::picture_csp_enc::linear_to_gamma` 0.2; `webp::enc::picture_csp_enc::convert_row_to_y` 0.1; `webp::enc::picture_csp_enc::import_yuva_from_rgba` 0.1 |
| quantize | 0.7 | `ferrisma::fast_execute::quantize_row::<3>` 0.7 |
