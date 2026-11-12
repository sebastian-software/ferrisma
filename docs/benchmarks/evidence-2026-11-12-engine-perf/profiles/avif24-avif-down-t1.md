# prof/avif24-avif-down-t1.json.gz: total CPU weight 3787283 (thread CPU delta units, us)

## Self (top 40)
| % | function |
| ---: | --- |
| 11.4 | `av1_optimize_txb` |
| 6.5 | `_platform_memset` |
| 5.2 | `avif::reformat::avifImageYUV8ToRGB8Color` |
| 4.9 | `av1_cnn_convolve_no_maxpool_padding_valid_c` |
| 4.9 | `od_ec_decode_cdf_q15` |
| 4.4 | `av1_highbd_quantize_fp_facade` |
| 3.5 | `ferrisma_kernels::dispatch::neon::horizontal_rgba` |
| 3.5 | `av1_read_coeffs_txb` |
| 2.9 | `<u8 as c2rust_bitfields::FieldType>::get_field` |
| 2.4 | `aom::av1__common__av1_loopfilter::set_lpf_parameters` |
| 1.6 | `aom::av1__encoder__encodeframe::highbd_compute_gradient_info_sb` |
| 1.5 | `__bzero` |
| 1.4 | `aom::av1__encoder__tx_search::search_tx_type` |
| 1.3 | `av1_predict_intra_block` |
| 1.2 | `av1_inv_txfm_add_c` |
| 1.2 | `aom::simd::av1__common__arm__highbd_inv_txfm_neon::idct32x32_neon` |
| 1.2 | `av1_fwd_txfm2d_8x8_neon` |
| 1.1 | `_platform_memmove` |
| 1.0 | `aom::av1__encoder__tx_search::dist_block_tx_domain` |
| 1.0 | `libsystem_malloc.dylib+?` |
| 1.0 | `aom::aom_dsp__loopfilter::filter8` |
| 0.9 | `aom_highbd_quantize_b_helper_c` |
| 0.9 | `aom::simd::av1__common__arm__highbd_inv_txfm_neon::inv_txfm2d_add_universe_neon` |
| 0.9 | `av1_cost_coeffs_txb` |
| 0.9 | `ferrisma::fast_execute::source_row_to_linear::<3>` |
| 0.9 | `aom::simd::av1__common__arm__highbd_inv_txfm_neon::idct8x8_neon` |
| 0.9 | `od_ec_decode_bool_q15` |
| 0.8 | `aom::simd::av1__common__arm__highbd_inv_txfm_neon::idct16x16_neon` |
| 0.8 | `av1_write_coeffs_txb` |
| 0.7 | `cfl_predict_hbd_8x8_c` |
| 0.7 | `avifImageRGBToYUV` |
| 0.7 | `aom::av1__common__av1_loopfilter::get_filter_level` |
| 0.6 | `aom::av1__encoder__txb_rdopt::get_coeff_cost_general` |
| 0.6 | `ferrisma_kernels::dispatch::neon::axpy` |
| 0.6 | `aom::av1__decoder__decodeframe::parse_decode_block` |
| 0.6 | `aom::simd::av1__encoder__arm__highbd_fwd_txfm_neon::highbd_fdct32_x4_neon` |
| 0.6 | `av1_get_nz_map_contexts_c` |
| 0.6 | `av1_nn_predict_c` |
| 0.5 | `av1_update_and_record_txb_context` |
| 0.5 | `aom::simd::av1__encoder__arm__highbd_fwd_txfm_neon::highbd_fdct16_x4_neon` |

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
| 100.0 | `ferrisma::pipeline::run` |
| 60.0 | `<ferrisma_avif_write::Frame>::finish` |
| 60.0 | `<ferrisma::own_avif::Frame as ferrisma::encoder_backend::RowEncoder>::finish` |
| 60.0 | `<ferrisma::pipeline::Sink>::finish` |
| 59.2 | `avif::write::avifEncoderAddImageInternal` |
| 59.2 | `avifEncoderWrite` |
| 59.1 | `avif::codec_aom::aomCodecEncodeImage` |
| 59.1 | `aom::av1__av1_cx_iface::encoder_encode` |
| 59.1 | `aom_codec_encode` |
| 59.0 | `aom::jump::value_scope::<u32, aom::av1__av1_cx_iface::encoder_encode::{closure#0}>` |
| 58.9 | `av1_encode_strategy` |
| 58.9 | `aom::jump::value_scope::<i32, aom::av1__encoder__encoder::av1_get_compressed_data::{closure#0}>` |
| 58.6 | `av1_encode` |
| 58.6 | `aom::av1__encoder__encoder::encode_with_recode_loop_and_filter` |
| 55.4 | `aom::av1__encoder__encodeframe::encode_frame_internal` |
| 55.4 | `av1_encode_frame` |
| 55.4 | `av1_encode_sb_row` |
| 55.4 | `av1_encode_tile` |
| 53.4 | `av1_rd_pick_partition` |
| 40.0 | `ferrisma::fast_execute::execute_fast_srgb8::<ferrisma::pipeline::run::{closure#1}>` |
| 39.7 | `<ferrisma::codecs::avif::AvifSource as ferrisma::decoder::DecodeSource>::read_region` |
| 36.1 | `aom::av1__encoder__partition_search::pick_sb_modes` |
| 35.9 | `av1_rd_pick_intra_mode_sb` |
| 33.9 | `<ferrisma_avif_read::Decoder>::decode_u8` |
| 28.7 | `aom::av1__av1_dx_iface::decoder_decode` |
| 28.7 | `aom_codec_decode` |
| 28.7 | `avif::codec_aom::aomCodecGetNextImage` |
| 28.7 | `avifDecoderNextImage` |
| 28.7 | `aom_decode_frame_from_obus` |
| 28.7 | `aom::jump::value_scope::<i32, aom::av1__decoder__decoder::av1_receive_compressed_data::{closure#0}>` |
| 28.7 | `av1_receive_compressed_data` |
| 28.7 | `aom::av1__av1_dx_iface::frame_worker_hook` |
| 28.7 | `aom::aom_util__aom_thread::execute` |

## Stage shares (leaf-to-root first match)
| stage | % CPU | top self functions in stage |
| --- | ---: | --- |
| aom-encode | 59.1 | `av1_optimize_txb` 11.4; `av1_cnn_convolve_no_maxpool_padding_valid_c` 4.9; `av1_highbd_quantize_fp_facade` 4.4; `_platform_memset` 3.7 |
| av1-decode | 28.7 | `od_ec_decode_cdf_q15` 4.9; `av1_read_coeffs_txb` 3.5; `_platform_memset` 2.7; `aom::av1__common__av1_loopfilter::set_lpf_parameters` 2.4 |
| yuv2rgb | 5.2 | `avif::reformat::avifImageYUV8ToRGB8Color` 5.2 |
| resize | 4.2 | `ferrisma_kernels::dispatch::neon::horizontal_rgba` 3.5; `ferrisma_kernels::dispatch::neon::axpy` 0.6 |
| linearize | 0.9 | `ferrisma::fast_execute::source_row_to_linear::<3>` 0.9 |
| rgb2yuv | 0.8 | `avifImageRGBToYUV` 0.7; `libsystem_m.dylib+?` 0.1; `ferrisma-base+?` 0.0 |
| engine-pipeline | 0.7 | `__bzero` 0.2; `libsystem_malloc.dylib+?` 0.2; `ferrisma::quality::taps` 0.1; `_platform_memmove` 0.1 |
| quantize | 0.4 | `ferrisma::fast_execute::quantize_row::<3>` 0.4 |
